// 这题看的题解，题解代码写得好难看 :(
#include <iostream>
#include <vector>
#include <algorithm>
#include <array>
using namespace std;

void Solution()
{
	int array_amount;
	cin >> array_amount;

	// 存储数组数据
	vector<vector<int>> arrays(array_amount);

	/*
	 * column_indeices[i] 用于记录在第i列有元素的数组编号
	 */
	vector<vector<int>> column_indices;

	int max_length{};

	// Input
	for (auto array_index = 0; array_index < array_amount; array_index++)
	{
		int current_array_length{};
		cin >> current_array_length;

		arrays[array_index].resize(current_array_length);
		max_length = max(max_length, current_array_length);

		for (auto column_index = 0; column_index < current_array_length; column_index++)
		{
			cin >> arrays[array_index][column_index];

			// 保证column_indices的列数不少于当前数列的列数
			while (column_indices.size() <= column_index)
			{
				column_indices.push_back({});
			}

			// 将当前数组放入列统计
			column_indices[column_index].push_back(array_index);
		}
	}

	/*
	 * column_min_value_index[i] 用于记录第i列元素中最小的那个元素的数列编号
	 */
	vector<int> column_min_value_index(max_length);

	/*
	 *	array_ranks[i] 表示第i个数列的排名，用于处理当两个数列根据数值排序相同时的情况
	 * 	-1 表示未排名
	 */
	vector<int> array_ranks(array_amount, -1);

	/*
	 * 从后往前进行列的处理，可以进行排名以处理前面完全相同的情况
	 */
	for (auto column_index = max_length - 1; column_index >= 0; column_index--)
	{
		/*
		 * current_column_data 用于存储当前列的数据(三元组): [值, 排名, 序列索引]
		 */
		vector<array<int, 3>> current_column_data;

		// 进行当前列数据的存储：遍历在当前列有元素的所有数组
		for (const auto array_index : column_indices[column_index])
		{
			current_column_data.push_back({
				arrays[array_index][column_index], // 当前列当前数组的值
				array_ranks[array_index],		   // 当前数组的排名(用于比较)
				array_index						   // 当前数组的索引
			});
		}

		/*
		 * 对当前列进行排序，排序顺序按[值, 排名, 序列索引]确定
		 * 从前往后值相同则判断下一个值。
		 *
		 * 使得: 值最小的放在最前，值相同时排名最好的在最前。
		 */
		sort(current_column_data.begin(), current_column_data.end());

		// 记录最小元素所在的数组索引
		column_min_value_index[column_index] = current_column_data[0][2];

		// 更新所有数组的排名: 根据当前列的排序结果
		int current_rank{};
		for (auto &data : current_column_data)
		{
			array_ranks[data[2]] = current_rank++;
		}
	}

	// 最终答案: 最底下最小的序列
	vector<int> result_array;

	// 没放满就一直放
	while (result_array.size() < max_length)
	{
		// 当前处理到的元素编号
		int handled_column_index = result_array.size();

		// 选择对于当前列来说最优的数组
		auto &selected_array = arrays[column_min_value_index[handled_column_index]];

		// 一直放入元素直到当前数组放完了
		for (int handling_column_index = handled_column_index; handling_column_index < selected_array.size(); handling_column_index++)
		{
			result_array.push_back(selected_array[handling_column_index]);
		}
	}

	// Output
	for (const auto element : result_array)
	{
		cout << element << ' ';
	}
	cout << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		Solution();
	}
	return 0;
}