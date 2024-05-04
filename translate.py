import os
from openai import OpenAI
from concurrent.futures import ThreadPoolExecutor
client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

source_folder = "./sv-benchmarks/java/jbmc-regression"
target_folder = "./examples/benchmark"

def migrate_file(file_path, subfolder_name):
    with open(file_path, "r") as f:
        content = f.read()

    prompt = f"""
    Migrate the following Java code to:

    1. Convert any usage of `import org.sosy_lab.sv_benchmarks.Verifier;` into function parameters.
    2. Change the class name (and file name) from `Main` to `{subfolder_name}`.
    3. Change the entry point function from `main(String[] args)` to `test` function with appropriate parameters.
    4. Remove any unecessary try-catch blocks that only lead to false assertions, but keep other fault assertions.

    You may return only code, without any additional text explanation, or markdown.

    Input Java code:

    {content}
    """

    response = client.chat.completions.create(model="gpt-4-turbo",
    messages=[
        {"role": "system", "content": "You are a helpful Java code assistant. "},
        {"role": "user", "content": prompt}
    ])

    return response.choices[0].message.content

def migrate_file_runner(file_path, subfolder_name):
    new_content = migrate_file(file_path, subfolder_name)

    new_content = new_content.replace("```java", "").replace("```", "")
    new_content = new_content.strip()

    if new_content[-1] != "\n":
        new_content += "\n"

    new_file_path = os.path.join(target_folder, subfolder_name, f"{subfolder_name}.java")
    os.makedirs(os.path.dirname(new_file_path), exist_ok=True)

    with open(new_file_path, "w") as f:
        f.write(new_content)

    print(f"Migrated {file_path} to {new_file_path}")

with ThreadPoolExecutor() as executor:
    for subdir, _, files in os.walk(source_folder):
        for file in files:
            if file != "Main.java":
                print(f"Skipping {file}")
                continue

            subfolder_name = os.path.basename(subdir)
            file_path = os.path.join(subdir, file)
            executor.submit(migrate_file_runner, file_path, subfolder_name)
