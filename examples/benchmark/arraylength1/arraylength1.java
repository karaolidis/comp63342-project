class arraylength1 {
  public static void test(int size) {
    if (size < 0) return;
    int int_array[] = new int[size];

    assert int_array.length == size;
  }
}
