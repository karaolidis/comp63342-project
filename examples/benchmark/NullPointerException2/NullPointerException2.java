class A {
  int i;
}

public class NullPointerException2 {
  public static void test(A a) {
    if (a == null) {
      assert false; // Assertion to handle null case explicitly
    } else {
      a.i = 0;
    }
  }
}
