class instanceof7 {
  public static void test(A[] as) {
    assert (!(as[0] instanceof B));
    assert (as[1] instanceof B);
  }
}

class A {
}

class B extends A {
}
