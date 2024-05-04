public class boolean2 {
  public static void test(boolean b) {
    boolean result = f(b);
    assert result == !b;
  }

  public static boolean f(boolean b) {
    return !b;
  }
}
