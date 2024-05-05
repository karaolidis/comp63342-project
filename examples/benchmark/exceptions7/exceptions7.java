class A extends RuntimeException {}

class B extends A {}

class C extends B {}

public class exceptions7 {
  public static void test() {
    try {
      try {
        C c = new C();
        A a = new A();
        throw a;
      } catch (C exc) {
        assert false;
      } catch (B exc) {
        assert false;
      }
    } catch (A exc) {
      assert false;
    }
  }
}
