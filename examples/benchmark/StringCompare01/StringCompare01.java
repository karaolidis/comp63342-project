public class StringCompare01 {
  public static void test(String s1, String s2, String s3, String s4) {
    if (s1.equals("test")) // true
      assert true;
    else
      assert false;

    if (s1 != "test") // false; s1 is initialized with "test"
      assert true;
    else
      assert false;

    if (s3.equalsIgnoreCase(s4)) // true
      assert true;
    else
      assert false;

    assert s1.compareTo(s2) == 13; // result depends on actual lexicographic comparison

    assert s2.compareTo(s1) == -13; // result depends on actual lexicographic comparison

    assert s1.compareTo(s1) == 0; // true

    assert s3.compareTo(s4) != 0; // true, since casing differs and it's case-sensitive

    assert s4.compareTo(s3) != 0; // true, corresponding to above comparison

    // test regionMatches (case sensitive)
    if (!s3.regionMatches(0, s4, 0, 5)) // true
      assert true;
    else
      assert false;

    // test regionMatches (ignore case)
    if (s3.regionMatches(true, 0, s4, 0, 5)) // true
      assert true;
    else
      assert false;
  }
}
