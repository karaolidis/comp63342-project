class A extends Throwable {}

class B extends A {}

class C extends B {}

class D extends C {}

public class exceptions5 {
  public static void test(D d, C c, B b, A a, A e) {
    try {
      throw e;
    } catch (D exc) {
      System.out.println("D");
      assert false;
    } catch (C exc) {
      System.out.println("C");
      assert false;
    } catch (B exc) {
      System.out.println("B");
      assert false;
    } catch (A exc) {
      System.out.println("A");
    }
  }

  public static void main(String[] args) {
    D d = new D();
    C c = new C();
    B b = new B();
    A a = new A();
    A e = a;
    test(d, c, b, a, e);
  }
}
