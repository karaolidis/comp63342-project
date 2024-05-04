class instanceof6 {
  public void test(A[] as) {
    assert (as[0] instanceof A);
    assert (as[1] instanceof A);
  }
}

class A {}

class B extends A {}
