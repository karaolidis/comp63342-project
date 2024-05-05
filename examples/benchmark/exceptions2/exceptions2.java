class A extends Throwable {}

class B extends A {}

class C extends B {}

public class exceptions2 {
  public static void test() {
    try {
      B b = new B();
      throw b;
    } catch (C exc) {
      assert false;
    } catch (B exc) {
      assert false;
    }
  }
}
