public class Synchronized {
  public static void test(Object o) {
    synchronized (o) {
    }
    assert o != null;
  }
}
