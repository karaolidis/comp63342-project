class A extends Throwable {
}

class B extends A {
}

class C extends B {
}

class D extends C {
}

public class exceptions5 {
  public void test() {
    try {
      D d = new D();
      C c = new C();
      B b = new B();
      A a = new A();
      A e = a;
      throw e;
    } catch (D exc) {
      System.out.println("D");
    } catch (C exc) {
      System.out.println("C");
    } catch (B exc) {
      System.out.println("B");
    } catch (A exc) {
      System.out.println("A");
    }
  }
}
