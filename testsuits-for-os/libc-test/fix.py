file_path = "./entry.h"
with open( file_path,'r') as open_file:
    content = open_file.read()
    content = content.replace("exe\n\"", "exe\"")
with open(file_path, 'w') as open_file:
    open_file.write(content)

with open( file_path,'r') as open_file:
    content = open_file.read()
    content = content.replace(".exe\n_main", "_main")
with open(file_path, 'w') as open_file:
    open_file.write(content)

with open( file_path,'r') as open_file:
    content = open_file.read()
    content = content.replace(".exe\"", "\"")
with open(file_path, 'w') as open_file:
    open_file.write(content)
