public class StringBuilderInsertDelete01 {
    public static void test(
        Object objectRef,
        String string,
        char[] charArray,
        boolean booleanValue,
        char characterValue,
        int integerValue,
        long longValue,
        float floatValue,
        double doubleValue
    ) {
        StringBuilder buffer = new StringBuilder();

        buffer
            .insert(0, objectRef)
            .insert(0, "-")
            .insert(0, string)
            .insert(0, "-")
            .insert(0, charArray)
            .insert(0, "-")
            .insert(0, charArray, 3, 3)
            .insert(0, "-")
            .insert(0, booleanValue)
            .insert(0, "-")
            .insert(0, characterValue)
            .insert(0, "-")
            .insert(0, integerValue)
            .insert(0, "-")
            .insert(0, longValue)
            .insert(0, "-")
            .insert(0, floatValue)
            .insert(0, "-")
            .insert(0, doubleValue);

        String tmp = buffer.toString();
        assert tmp.equals("33.333-2.5-10000000-7-K-true-ifi-verification-test-diffblue");

        buffer.deleteCharAt(10);
        buffer.delete(2, 6);

        tmp = buffer.toString();
        assert tmp.equals("33-2.510000000-7-K-true-ifi-verification-test-diffblue");
    }
}
