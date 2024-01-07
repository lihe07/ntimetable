use roaring::bitmap::RoaringBitmap;

fn main() {
    let mut map = RoaringBitmap::new();

    map.insert(1);
    map.insert(1);
    map.insert(2);
    map.insert(3);

    let mut map2 = RoaringBitmap::new();
    map2.insert(3);
    map2.insert(2);

    dbg!(&map, &map2);

    dbg!(map & map2);
}
