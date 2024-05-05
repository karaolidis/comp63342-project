class A extends Exception {}

class athrow1 {
  public static void test() {
    A a = new A();
    try {
      throw a;
    } catch (Exception e) {
      assert false;
    }
  }
}
