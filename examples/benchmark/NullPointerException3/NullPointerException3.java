class A {
  int i;
}

public class NullPointerException3 {
  public static void test() {
    A a = null;
    try {
      int i = a.i;
    } catch (NullPointerException exc) {
      assert false;
    }
  }
}
