/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/jbmc-strings/StringConcatenation02
 * The benchmark was taken from the repo: 24 January 2018
 */
public class StringConcatenation02 {
  public static void test(String s1, String s2) {
    assert s1.equals(s1 + " ");
    assert s2.equals(s2);
  }
}
