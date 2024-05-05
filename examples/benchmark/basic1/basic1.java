/* Hello World Java Program modified to basic1 */
public class basic1 {
  public static void test() {
    assert (System.out != null);
    System.out.println("Hello World!");
    assert (System.err != null);
    System.err.println("Hello World!");
    assert (System.in != null);
    try {
      int avail = System.in.available();
    } catch (java.io.IOException e) {
    }
  }
}
