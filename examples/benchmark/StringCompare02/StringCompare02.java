public class StringCompare02 {
  public static void test(String s3, String s4) {
    // test regionMatches (case sensitive)
    if (s3.regionMatches(0, s4, 0, 5)) // false
    assert true;
    else assert false;
  }
}
