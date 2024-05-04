class if_expr1 {
  public static void test(int x) {
    int y = x == 10 ? 11 : 9;
    if (x == 10)
      assert y == 11;
    else
      assert y == 9;
  }
}
