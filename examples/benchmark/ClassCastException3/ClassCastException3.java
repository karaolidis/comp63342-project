class A {}

class B {}

public class ClassCastException3 {
  public static void test() {
    try {
      Object a = new A();
      B b = (B) a;
    } catch (Exception exc) {
      assert false;
    }
  }
}
