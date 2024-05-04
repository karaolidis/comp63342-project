class A extends Throwable {
}

class B extends A {
}

class C extends B {
}

public class exceptions4 {
  public void test(B b) {
    try {
      throw b;
    } catch (C exc) {
      System.out.println("C");
      assert false;
    } catch (B exc) {
      System.out.println("B");
    }
  }
}
