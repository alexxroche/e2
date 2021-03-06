"""
rdfa:src="https://gist.github.com/reimund/5435343/"
rdfa:ref="https://stackoverflow.com/a/16149359"
	Simple xml serializer.

	@author Reimund Trost 2013

	Example:
	 
	mydict = {
		'name': 'The Andersson\'s',
		'size': 4,
		'children': {
			'total-age': 62,
			'child': [
				{ 'name': 'Tom', 'sex': 'male', },
				{
					'name': 'Betty',
					'sex': 'female',
					'grandchildren': {
						'grandchild': [
							{ 'name': 'herbert', 'sex': 'male', },
							{ 'name': 'lisa', 'sex': 'female', }
						]
					},
				}
			]
		},
	}
	 
	print(dict2xml(mydict, 'family'))
	 
	Output:
	  
	  <family name="The Andersson's" size="4">
		<children total-age="62">
		  <child name="Tom" sex="male"/>
		  <child name="Betty" sex="female">
			<grandchildren>
			  <grandchild name="herbert" sex="male"/>
			  <grandchild name="lisa" sex="female"/>
			</grandchildren>
		  </child>
		</children>
	  </family>
"""
def dict2xml(d, root_node=None):
	wrap          =     False if None == root_node or isinstance(d, list) else True
	root          = 'objects' if None == root_node else root_node
	if type(root) is str:
		root_singular = root[:-1] if 's' == root[-1] and None == root_node else root
	else:
		root_singular = root

	xml           = ''
	children      = []

	if isinstance(d, dict):
		for key, value in dict.items(d):
			if isinstance(value, dict):
				children.append(dict2xml(value, key))
			elif isinstance(value, list):
				children.append(dict2xml(value, key))
			else:
				xml = xml + ' ' + key + '="' + str(value) + '"'
				#xml = '%s %s="%s"' % (xml,key,value)
				#xml = f'{xml} {key}="{value}"'
	else:
		for value in d:
			children.append(dict2xml(value, root_singular))

	end_tag = '>' if 0 < len(children) else '/>'

	if wrap or isinstance(d, dict):
		xml = '<' + root + xml + end_tag

	if 0 < len(children):
		for child in children:
			xml = xml + child

		if wrap or isinstance(d, dict):
			xml = xml + '</' + root + '>'
		
	return xml

