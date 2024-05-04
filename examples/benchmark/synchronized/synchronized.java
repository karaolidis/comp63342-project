public class Synchronized {
  public void test(Object o) {
    synchronized (o) {
    }
    assert o != null;
  }
}
