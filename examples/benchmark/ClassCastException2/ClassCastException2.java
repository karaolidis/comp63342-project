class A {}

class B extends A {}

class C extends B {}

public class ClassCastException2 {
  public static void test() {
    try {
      A c = new C();
      B b = (B) c;
    } catch (ClassCastException exc) {
      assert false;
    }
  }
}
