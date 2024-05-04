class A {
  public int i;
}

class putfield_getfield1 {
  public static void test(A a) {
    a.i = 999;
    assert 999 == a.i;
  }
}
