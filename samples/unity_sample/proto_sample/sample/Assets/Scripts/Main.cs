using Gable;
using UnityEngine;
using UnityEngine.UI;

public class Main : MonoBehaviour
{
    [SerializeField]
    private Dropdown _dropdown;

    [SerializeField]
    private Button _read;

    // Start is called before the first frame update
    private void Start()
    {
        TableManager.Instance.Load();
        _read.onClick.AddListener(OnClickRead);
    }

    // Update is called once per frame
    private void OnClickRead()
    {
        switch (_dropdown.captionText.text)
        {
            case "Const":
                Gable.Vector3 pos = TableManager.Instance.Const.player_default_position;
                Debug.Log($"x:{pos.x}, y:{pos.y}, z:{pos.z}");
                break;

            case "Player":
                Player player = TableManager.Instance.GetPlayer(1002);
                Debug.Log($"id:{player.id}, type:{player.type}, name:{player.name}, cd:{player.cd}");
                break;

            case "PlayerLevel":
                PlayerLevel leveConf = TableManager.Instance.GetPlayerLevel(1001,1);
                Debug.Log($"id:{leveConf.id}, level:{leveConf.level}, attack:{leveConf.attack}, defense:{leveConf.defense}, hp:{leveConf.hp}");
                break;
        }
    }
}