var searchIndex = {};
searchIndex["panser"] = {"doc":"Panser","items":[[3,"Panser","panser","A Builder for transcoding.",null,null],[4,"Framing","","",null,null],[13,"Sized","","",0,null],[13,"Delimited","","",0,null],[4,"ToFormat","","",null,null],[13,"Bincode","","",1,null],[13,"Cbor","","",1,null],[13,"Hjson","","",1,null],[13,"Json","","",1,null],[13,"Msgpack","","",1,null],[13,"Pickle","","",1,null],[13,"Toml","","",1,null],[13,"Url","","",1,null],[13,"Yaml","","",1,null],[4,"FromFormat","","",null,null],[13,"Bincode","","",2,null],[13,"Cbor","","",2,null],[13,"Envy","","",2,null],[13,"Hjson","","",2,null],[13,"Json","","",2,null],[13,"Msgpack","","",2,null],[13,"Pickle","","",2,null],[13,"Toml","","",2,null],[13,"Url","","",2,null],[13,"Yaml","","",2,null],[4,"Radix","","",null,null],[13,"Binary","","",3,null],[13,"Decimal","","",3,null],[13,"Hexadecimal","","",3,null],[13,"Octal","","",3,null],[4,"Error","","",null,null],[13,"Bincode","","",4,null],[13,"Cbor","","",4,null],[13,"Eof","","",4,null],[13,"Generic","","",4,null],[13,"Io","","",4,null],[13,"Json","","",4,null],[13,"MsgpackDecode","","",4,null],[13,"MsgpackEncode","","",4,null],[13,"ParseInt","","",4,null],[13,"Pickle","","",4,null],[13,"TomlDecode","","",4,null],[13,"TomlEncode","","",4,null],[13,"Utf8","","",4,null],[13,"UrlDecode","","",4,null],[13,"UrlEncode","","",4,null],[13,"Yaml","","",4,null],[5,"deserialize","","Deserialize to a universal, arbitrary value.",null,null],[5,"serialize","","Serialize from a universal, arbitrary value.",null,{"inputs":[{"name":"value"},{"name":"toformat"}],"output":{"name":"result"}}],[5,"transcode","","Convert the input in one format to the output of another format.",null,null],[11,"new","","Creates a new `Panser` with default options.",5,{"inputs":[],"output":{"name":"panser"}}],[11,"delimited_input","","Sets a delimiter byte for the input and changes to framed reading of the data.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"delimited_output","","Sets a delimiter byte for the output.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"from","","The format of the input.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"inputs","","The input source.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"output","","The output destination.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"radix","","Sets the written output to be a space-separated list of bytes represented as numeric strings with a specific radix, or notation.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[11,"run","","Create a producer-consumer architecture for reading and writing data. ",5,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"sized_input","","Indicates the first four bytes is the total data length and changes to framed reading of the data.",5,{"inputs":[{"name":"self"},{"name":"bool"}],"output":{"name":"self"}}],[11,"sized_output","","Prepends the length of the data to the output.",5,{"inputs":[{"name":"self"},{"name":"bool"}],"output":{"name":"self"}}],[11,"to","","The format of the output.",5,{"inputs":[{"name":"self"},{"name":"option"}],"output":{"name":"self"}}],[6,"Result","","",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"framing"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"toformat"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"possible_values","","",1,{"inputs":[],"output":{"name":"vec"}}],[11,"from_str","","",1,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"fromformat"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"possible_values","","",2,{"inputs":[],"output":{"name":"vec"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",2,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"radix"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"possible_values","","",3,{"inputs":[],"output":{"name":"vec"}}],[11,"from_str","","",3,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"code","","",4,{"inputs":[{"name":"self"}],"output":{"name":"i32"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"box"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"parseinterror"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"utf8error"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"error"}}]],"paths":[[4,"Framing"],[4,"ToFormat"],[4,"FromFormat"],[4,"Radix"],[4,"Error"],[3,"Panser"]]};
initSearch(searchIndex);
