class assert1 {
  public static void test(int i) {
    if (i >= 10)
      assert i >= 10 : "my super assertion"; // should hold

    if (i >= 20)
      assert i >= 20 : "my super assertion"; // should hold
  }
}
