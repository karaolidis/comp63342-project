class if_icmp1 {
  public static void f(int i, int j) {
    if (i == j) {
      assert false;
    }
    if (i >= j) {
      assert false;
    }
    if (j > i) {
      assert true;
    } else {
      assert false;
    }
    if (j <= i) {
      assert false;
    }
    if (j < i) {
      assert false;
    } else {
      assert true;
    }
  }

  public static void test(int i) {
    if (i + 1 < 0) return;
    f(i, i + 1);
  }
}
