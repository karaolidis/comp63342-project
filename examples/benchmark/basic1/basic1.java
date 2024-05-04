class basic1 {
  public static void test() {
    assert (System.out != null);
    System.out.println("Hello World!");
    assert (System.err != null);
    System.err.println("Hello World!");
    assert (System.in != null);
    int avail = 0;
    avail = System.in.available();
  }
}
