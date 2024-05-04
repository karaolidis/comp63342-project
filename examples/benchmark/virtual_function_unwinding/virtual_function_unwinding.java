interface Interface {
  boolean b();
}

class Inner implements Interface {
  public boolean b() {
    return false;
  }
}

class Outer implements Interface {
  private Interface inner;

  public Outer(Interface inner) {
    this.inner = inner;
  }

  public boolean b() {
    return !inner.b();
  }
}

public class virtual_function_unwinding {
  public static void test() {
    assert new Outer(new Inner()).b();
  }
}
