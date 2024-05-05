class some_exception1 extends Throwable {}
;

class some_exception2 extends some_exception1 {}
;

class catch1 {
  public static void test() {
    try {
      throw new some_exception2();
    } catch (some_exception1 e) {
      assert e instanceof some_exception2;
      if (e instanceof some_exception2) return;
    }
    assert false;
  }
}
