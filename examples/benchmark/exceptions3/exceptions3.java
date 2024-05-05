class A extends Throwable {}

class B extends A {}

class C extends B {}

public class exceptions3 {
  public static void test() {
    try {
      throw new B();
    } catch (C exc) {
      assert false;
    } catch (B exc) {
      assert false;
    }
  }
}
