class assert3 {
  public static void test(int i) {
    if (i >= 1000)
      if (!(i > 1000))
        assert false;
  }
}
