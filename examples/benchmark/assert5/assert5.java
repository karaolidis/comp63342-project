class assert5 {
  public static void test(int i) {
    if (i > 1000) assert i > 1000 : "i is greater 1000"; // should hold
  }
}
