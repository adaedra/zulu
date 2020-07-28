use zulu::Zfs;

#[test]
fn test_init_and_close() {
    Zfs::new().unwrap();
}
