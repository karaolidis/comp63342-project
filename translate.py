import os
from openai import OpenAI
from concurrent.futures import ThreadPoolExecutor
client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

source_folder = "./sv-benchmarks/java/jbmc-regression"
target_folder = "./examples/benchmark"

def migrate_file(content, subfolder_name):
    prompt = f"""
    Migrate the following Java code to:

    1. Convert any usage of `import org.sosy_lab.sv_benchmarks.Verifier;` into function parameters.
    2. Change the class name (and file name) from `Main` to `{subfolder_name}`.
    3. Change the entry point function from `main(String[] args)` to `test` function with appropriate parameters.
    4. Change the entry point function and any other functions to public static if they aren't already, as long as it doesn't change the semantics.

    You may return only code, without any additional text explanation, or markdown.

    Input Java code:

    ```java
    {content}
    ```
    """

    response = client.chat.completions.create(model="gpt-4-turbo",
    messages=[
        {"role": "system", "content": "You are a helpful Java code assistant. "},
        {"role": "user", "content": prompt}
    ])

    return response.choices[0].message.content

def migrate_file_runner(content, subfolder_name):
    new_content = migrate_file(content, subfolder_name)

    new_content = new_content.replace("```java", "").replace("```", "")
    new_content = new_content.strip()

    if new_content[-1] != "\n":
        new_content += "\n"

    new_file_path = os.path.join(target_folder, subfolder_name, f"{subfolder_name}.java")
    os.makedirs(os.path.dirname(new_file_path), exist_ok=True)

    with open(new_file_path, "w") as f:
        f.write(new_content)

with ThreadPoolExecutor() as executor:
    for subdir, _, files in os.walk(source_folder):
        content = ""

        for file in files:
            if not file.endswith(".java"):
                continue

            with open(os.path.join(subdir, file), "r") as f:
                text = f.read()

            text = text.split("*/", 1)

            if len(text) > 1:
                text = text[1]
            else:
                text = text[0]

            content += text
            content += "\n"

        if content:
            subfolder_name = os.path.basename(subdir)
            executor.submit(migrate_file_runner, content, subfolder_name)
