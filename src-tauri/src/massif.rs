use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize, Clone)]
pub struct HeapNode {
    pub num_children: u32,
    pub bytes: u64,
    pub address: String,
    pub function: String,
    pub file_info: Option<String>,
    pub children: Vec<HeapNode>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Snapshot {
    pub snapshot_num: u32,
    pub time: u64,
    pub mem_heap_b: u64,
    pub mem_heap_extra_b: u64,
    pub mem_stacks_b: u64,
    pub heap_tree: Option<HeapNode>,
}

#[derive(Debug, Serialize)]
pub struct MassifData {
    pub desc: String,
    pub cmd: String,
    pub time_unit: String,
    pub snapshots: Vec<Snapshot>,
}

pub fn parse_massif_file(path: &str) -> Result<MassifData, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse_massif_content(&content)
}

fn parse_massif_content(content: &str) -> Result<MassifData, String> {
    let mut desc = String::new();
    let mut cmd = String::new();
    let mut time_unit = String::new();
    let mut snapshots: Vec<Snapshot> = Vec::new();

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("desc:") {
            desc = line.strip_prefix("desc:").unwrap_or("").trim().to_string();
        } else if line.starts_with("cmd:") {
            cmd = line.strip_prefix("cmd:").unwrap_or("").trim().to_string();
        } else if line.starts_with("time_unit:") {
            time_unit = line
                .strip_prefix("time_unit:")
                .unwrap_or("")
                .trim()
                .to_string();
        } else if line.starts_with("snapshot=") {
            let snapshot_num: u32 = line
                .strip_prefix("snapshot=")
                .unwrap_or("0")
                .parse()
                .unwrap_or(0);

            i += 1;
            // Skip separator line
            while i < lines.len() && lines[i].trim().starts_with('#') {
                i += 1;
            }

            let mut time: u64 = 0;
            let mut mem_heap_b: u64 = 0;
            let mut mem_heap_extra_b: u64 = 0;
            let mut mem_stacks_b: u64 = 0;
            let mut heap_tree: Option<HeapNode> = None;

            while i < lines.len() {
                let snap_line = lines[i].trim();

                if snap_line.starts_with('#') || snap_line.starts_with("snapshot=") {
                    break;
                }

                if snap_line.starts_with("time=") {
                    time = snap_line
                        .strip_prefix("time=")
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                } else if snap_line.starts_with("mem_heap_B=") {
                    mem_heap_b = snap_line
                        .strip_prefix("mem_heap_B=")
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                } else if snap_line.starts_with("mem_heap_extra_B=") {
                    mem_heap_extra_b = snap_line
                        .strip_prefix("mem_heap_extra_B=")
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                } else if snap_line.starts_with("mem_stacks_B=") {
                    mem_stacks_b = snap_line
                        .strip_prefix("mem_stacks_B=")
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                } else if snap_line.starts_with("heap_tree=detailed") {
                    i += 1;
                    let (tree, new_i) = parse_heap_tree(&lines, i, 0);
                    heap_tree = tree;
                    i = new_i;
                    continue;
                } else if snap_line.starts_with("heap_tree=empty") {
                    // No tree data
                }

                i += 1;
            }

            snapshots.push(Snapshot {
                snapshot_num,
                time,
                mem_heap_b,
                mem_heap_extra_b,
                mem_stacks_b,
                heap_tree,
            });

            continue;
        }

        i += 1;
    }

    Ok(MassifData {
        desc,
        cmd,
        time_unit,
        snapshots,
    })
}

fn parse_heap_tree(lines: &[&str], start: usize, expected_indent: usize) -> (Option<HeapNode>, usize) {
    if start >= lines.len() {
        return (None, start);
    }

    let line = lines[start];
    let trimmed = line.trim();

    // Check if this is a heap tree node line (starts with 'n')
    if !trimmed.starts_with('n') {
        return (None, start);
    }

    // Calculate current line's indentation
    let current_indent = line.len() - line.trim_start().len();

    // If indentation is less than expected, we've moved up in the tree
    if current_indent < expected_indent && expected_indent > 0 {
        return (None, start);
    }

    // Parse the node: format is "nX: BYTES ADDRESS: FUNCTION (FILE:LINE)" or "nX: BYTES in Y places..."
    let node = parse_heap_node_line(trimmed);

    if let Some(mut node) = node {
        let mut i = start + 1;

        // Parse children based on num_children
        while node.children.len() < node.num_children as usize && i < lines.len() {
            let (child, new_i) = parse_heap_tree(lines, i, current_indent + 1);
            if let Some(child) = child {
                node.children.push(child);
                i = new_i;
            } else {
                break;
            }
        }

        return (Some(node), i);
    }

    (None, start + 1)
}

fn parse_heap_node_line(line: &str) -> Option<HeapNode> {
    // Format: "nX: BYTES ADDRESS: FUNCTION (FILE:LINE)"
    // or: "nX: BYTES in Y places, all below massif's threshold (Z%)"

    if !line.starts_with('n') {
        return None;
    }

    // Find the colon after nX
    let colon_pos = line.find(':')?;
    let num_children: u32 = line[1..colon_pos].parse().ok()?;

    let rest = line[colon_pos + 1..].trim();

    // Parse bytes (first number)
    let parts: Vec<&str> = rest.splitn(2, ' ').collect();
    if parts.is_empty() {
        return None;
    }

    let bytes: u64 = parts[0].parse().unwrap_or(0);

    // Check if it's "in X places" format
    if parts.len() > 1 && parts[1].starts_with("in ") {
        return Some(HeapNode {
            num_children,
            bytes,
            address: String::new(),
            function: rest.to_string(),
            file_info: None,
            children: Vec::new(),
        });
    }

    // Parse address and function
    let mut address = String::new();
    let mut function = String::new();
    let mut file_info: Option<String> = None;

    if parts.len() > 1 {
        let after_bytes = parts[1];

        // Try to find address (starts with 0x)
        if let Some(addr_end) = after_bytes.find(": ") {
            address = after_bytes[..addr_end].trim().to_string();
            let func_part = &after_bytes[addr_end + 2..];

            // Check for file info in parentheses
            if let Some(paren_start) = func_part.rfind(" (") {
                function = func_part[..paren_start].to_string();
                let paren_content = &func_part[paren_start + 2..];
                if let Some(paren_end) = paren_content.rfind(')') {
                    file_info = Some(paren_content[..paren_end].to_string());
                }
            } else {
                function = func_part.to_string();
            }
        } else {
            function = after_bytes.to_string();
        }
    }

    Some(HeapNode {
        num_children,
        bytes,
        address,
        function,
        file_info,
        children: Vec::new(),
    })
}
