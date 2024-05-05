class A {
  int i;
}

public class NullPointerException2 {
  public static void test(A a) {
    try {
      a.i = 0;
    } catch (NullPointerException exc) {
      assert false;
    }
  }
}
