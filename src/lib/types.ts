export interface HeapNode {
  num_children: number;
  bytes: number;
  address: string;
  function: string;
  file_info: string | null;
  children: HeapNode[];
}

export interface Snapshot {
  snapshot_num: number;
  time: number;
  mem_heap_b: number;
  mem_heap_extra_b: number;
  mem_stacks_b: number;
  heap_tree: HeapNode | null;
}

export interface MassifData {
  desc: string;
  cmd: string;
  time_unit: string;
  snapshots: Snapshot[];
}
