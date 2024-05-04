/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/jbmc-strings/StringCompare05
 * The benchmark was taken from the repo: 24 January 2018
 */
public class StringCompare05 {
  public void test(String string) {
    String s1 = new String(string);
    assert s1 != string; // ensuring they are not the same object
  }
}
