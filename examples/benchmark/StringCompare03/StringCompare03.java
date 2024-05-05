public class StringCompare03 {
  public static void test(String s3, String s4) {
    // test regionMatches (ignore case)
    if (!s3.regionMatches(true, 0, s4, 0, 5)) // false
    assert true;
    else assert false;
  }
}
