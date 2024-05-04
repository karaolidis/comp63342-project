class assert4 {
  public static void test(int i) {
    if (i >= 10)
      assert i >= 20 : "my super assertion"; // should hold
  }
}
