pub(crate) enum FileType {
    MutLog, // active file for write
    ImmLog, // immutable file for read only

    Hint,     // persisit index for speeding recover
    Manifest, // the manifest to manage the whole db and compaction
}
