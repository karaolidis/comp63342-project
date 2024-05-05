public class bitwise1 {

  static char c;

  public static void test(int nondetInt) {
    c = (char) (nondetInt * 2 + 1);
    int i = (c | 2);
    assert (i & 3) == 3;
  }
}
