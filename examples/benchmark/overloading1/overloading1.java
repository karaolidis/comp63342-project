class other_class {}

class overloading1 {
  public static void test(int i, double d, other_class o) {
    assert (f(i) == 1);
    assert (f(d) == 2);
    assert (f(o) == 3);
  }

  static int f(int i) {
    return 1;
  }

  static int f(double d) {
    return 2;
  }

  static int f(other_class o) {
    return 3;
  }
}
