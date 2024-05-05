class A extends RuntimeException {}

class B extends A {}

class C extends B {}

public class exceptions16 {
  public static void test(int nondetX) {
    int x = nondetX;
    try {
      x++;
    } catch (A exc) {
      assert false;
    }

    try {
      throw new B();
    } catch (B exc) {
      assert false;
    }
  }
}
