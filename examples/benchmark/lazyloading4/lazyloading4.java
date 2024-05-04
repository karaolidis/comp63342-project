class Problem {
  private static final Object[] DEFAULT = {};
  private Object data;

  Problem() {
    this.data = DEFAULT;
  }

  void checkInvariant() {
    assert data != null;
  }
}

public class lazyloading4 {
  public void test() {
    new Problem().checkInvariant();
  }
}
