class fcmpx_dcmpx1 {
  public static void test(float f, double d) {
    assert f == 2.5f;
    assert f < 3.0f;
    assert f > 2.0f;
    assert d == 2.5;
    assert d < 3.0;
    assert d > 2.0;
  }
}
