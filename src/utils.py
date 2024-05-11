from werkzeug.datastructures import FileStorage
import hashlib

def handle_upload_file(file_storage: FileStorage) -> str:
    import os

    if isinstance(file_storage, FileStorage):
        file_name = file_storage.filename
        file_suffix = ""
        if file_name is not None:
            dot_index = file_name.rfind(".")
            file_suffix = file_name[dot_index:len(file_name)]
        data = file_storage.stream.read()
        file_storage.stream.seek(0)
        md5 = hashlib.md5(data)
        file_name = md5.hexdigest() + file_suffix
        file_path = f"./dist/uploads/{file_name}"
        if not os.path.exists(file_path):
            print(f"Storing file {file_path}")
            file_storage.save(file_path)
        file_storage.close()

        return file_path
