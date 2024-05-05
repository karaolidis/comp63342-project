class A {
  int i;
}

public class NullPointerException4 {
  public static void test(A a) {
    try {
      a.i = 0;
    } catch (Exception exc) {
      assert false;
    }
  }
}
