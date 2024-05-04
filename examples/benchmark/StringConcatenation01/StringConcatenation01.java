/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/jbmc-strings/StringConcatenation01
 * The benchmark was taken from the repo: 24 January 2018
 */
public class StringConcatenation01 {
  public static void test(String s1, String s2) {
    assert s1.equals(s1);
    assert s2.equals(s2);

    String tmp = s1.concat(s2);
    assert tmp.equals(s1 + s2);

    tmp = s1;
    assert tmp.equals(s1);
  }
}
