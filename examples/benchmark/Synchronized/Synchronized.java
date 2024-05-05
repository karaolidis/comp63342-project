public class Synchronized {
  public static void test() {
    final Object o = null;
    try {
      synchronized (o) {
      }
      assert false;
    } catch (NullPointerException e) {
      return;
    }
  }
}
