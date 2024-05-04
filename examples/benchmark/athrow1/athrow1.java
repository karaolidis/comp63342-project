class A extends Exception {
}

class athrow1 {
  public static void test(A a) {
    throw a;
  }
}
